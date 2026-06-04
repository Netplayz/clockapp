#include "tos.HH"

I64 F64ToI64(F64 x) { F64 ip; modf(x, &ip); return lrint(ip); }

I64 ParseInt(U8 *s, I64 *pos) {
    I64 val = 0;
    while (s[*pos] == 0x20 || s[*pos] == 0x54 || s[*pos] == 0x3A || s[*pos] == 0x2D) { (*pos)++; }
    if (s[*pos] == 0x2D) { (*pos)++; }
    while (s[*pos] >= 0x30 && s[*pos] <= 0x39) {
        val = val * 10 + (s[*pos] - 0x30);
        (*pos)++;
    }
    return val;
}

F64 ParseFlt(U8 *s, I64 *pos) {
    F64 val = 0.0, frac = 0.0, div = 10.0;
    I64 neg = 0;
    while (s[*pos] == 0x20 || s[*pos] == 0x54 || s[*pos] == 0x3A || s[*pos] == 0x2D) { (*pos)++; }
    if (s[*pos] == 0x2D) { neg = 1; (*pos)++; }
    while (s[*pos] >= 0x30 && s[*pos] <= 0x39) {
        val = val * 10.0 + (s[*pos] - 0x30);
        (*pos)++;
    }
    if (s[*pos] == 0x2E) {
        (*pos)++;
        while (s[*pos] >= 0x30 && s[*pos] <= 0x39) {
            frac += (s[*pos] - 0x30) / div;
            div *= 10.0;
            (*pos)++;
        }
    }
    if (neg) { return -(val + frac); }
    return (val + frac);
}

F64 Deg2Rad(F64 d) { return d * 3.14159265358979323846 / 180.0; }
F64 Rad2Deg(F64 r) { return r * 180.0 / 3.14159265358979323846; }

F64 CalcJD(I64 y, I64 m, I64 d, F64 h) {
    I64 a, b;
    if (m <= 2) { y--; m += 12; }
    a = y / 100;
    b = 2 - a + a / 4;
    return trunc(365.25 * (y + 4716)) + trunc(30.6001 * (m + 1)) + d + b - 1524.5 + h / 24.0;
}

U0 Main(I64 argc, U8 **argv) {
    F64 lat, lon, jd, n, M_deg, L_deg, C_deg, lambda_deg, eps_deg, delta_deg;
    F64 alpha_deg, EoT, elev, ha_now_rad, jtransit, sunrise, sunset, daylen;
    I64 y, mo, d, hh, mm, pos;
    F64 ss, hour_dec, intpart;
    U8 *dt;

    if (argc < 4) { printf("Usage: solar_calc lat lon datetime\n"); return; }

    pos = 0; lat = ParseFlt(argv[1], &pos);
    pos = 0; lon = ParseFlt(argv[2], &pos);
    dt = argv[3];

    pos = 0; y = ParseInt(dt, &pos);
    mo = ParseInt(dt, &pos);
    d = ParseInt(dt, &pos);
    hh = ParseInt(dt, &pos);
    mm = ParseInt(dt, &pos);
    pos++; ss = ParseFlt(dt, &pos);

    hour_dec = hh + mm / 60.0 + ss / 3600.0;
    jd = CalcJD(y, mo, d, hour_dec);
    n = jd - 2451545.0;

    M_deg = 357.5291 + 0.98560028 * n;
    L_deg = 280.460 + 0.9856474 * n;
    M_deg = fmod(M_deg, 360.0);
    L_deg = fmod(L_deg, 360.0);
    if (M_deg < 0) M_deg += 360.0;
    if (L_deg < 0) L_deg += 360.0;

    C_deg = 1.9148 * sin(Deg2Rad(M_deg)) + 0.0200 * sin(Deg2Rad(2 * M_deg)) + 0.0003 * sin(Deg2Rad(3 * M_deg));
    lambda_deg = L_deg + C_deg;
    lambda_deg = fmod(lambda_deg, 360.0);
    if (lambda_deg < 0) lambda_deg += 360.0;

    eps_deg = 23.439 - 0.0000004 * n;
    delta_deg = Rad2Deg(asin(sin(Deg2Rad(eps_deg)) * sin(Deg2Rad(lambda_deg))));

    alpha_deg = Rad2Deg(atan2(cos(Deg2Rad(eps_deg)) * sin(Deg2Rad(lambda_deg)), cos(Deg2Rad(lambda_deg))));
    EoT = 4.0 * (L_deg - 0.0057183 - alpha_deg);
    if (EoT > 180.0) EoT -= 360.0;
    if (EoT < -180.0) EoT += 360.0;

    F64 lat_r = Deg2Rad(lat);
    F64 cos_lat = cos(lat_r), sin_lat = sin(lat_r);
    F64 cos_dec = cos(Deg2Rad(delta_deg)), sin_dec = sin(Deg2Rad(delta_deg));
    F64 cos_ha = (-sin(Deg2Rad(-0.83)) - sin_lat * sin_dec) / (cos_lat * cos_dec);
    if (cos_ha > 1.0) cos_ha = 1.0;
    if (cos_ha < -1.0) cos_ha = -1.0;
    F64 ha_deg = Rad2Deg(acos(cos_ha));

    jtransit = 2451545.0 + n + 0.5 - lon / 360.0;
    sunrise = jtransit - ha_deg / 360.0;
    sunset = jtransit + ha_deg / 360.0;
    daylen = 2.0 * ha_deg / 15.0;

    ha_now_rad = Deg2Rad(15.0 * (hour_dec - 12.0) - lon);
    elev = Rad2Deg(asin(sin_lat * sin_dec + cos_lat * cos_dec * cos(ha_now_rad)));

    modf(jtransit + 0.5, &intpart);
    F64 midnight_jd = intpart - 0.5;
    F64 sn_frac = (jtransit - midnight_jd) * 24.0;
    modf(sn_frac, &intpart);
    I64 sn_h = lrint(intpart);
    modf((sn_frac - intpart) * 60, &intpart);
    I64 sn_m = lrint(intpart);

    modf(sunrise + 0.5, &intpart);
    midnight_jd = intpart - 0.5;
    F64 sv_frac = (sunrise - midnight_jd) * 24.0;
    if (sv_frac < 0) sv_frac += 24.0;
    if (sv_frac >= 24) sv_frac -= 24.0;
    modf(sv_frac, &intpart);
    I64 sv_h = lrint(intpart);
    F64 sv_rem = (sv_frac - intpart) * 60;
    modf(sv_rem, &intpart);
    I64 sv_m = lrint(intpart);
    I64 sv_s = lrint((sv_rem - intpart) * 60);

    modf(sunset + 0.5, &intpart);
    midnight_jd = intpart - 0.5;
    F64 ss_frac = (sunset - midnight_jd) * 24.0;
    if (ss_frac < 0) ss_frac += 24.0;
    if (ss_frac >= 24) ss_frac -= 24.0;
    modf(ss_frac, &intpart);
    I64 ss_h = lrint(intpart);
    F64 ss_rem = (ss_frac - intpart) * 60;
    modf(ss_rem, &intpart);
    I64 ss_m = lrint(intpart);
    I64 ss_s = lrint((ss_rem - intpart) * 60);

    printf("SV:%04lld-%02lld-%02lldT%02lld:%02lld:%02lld\n", y, mo, d, sv_h, sv_m, sv_s);
    printf("SS:%04lld-%02lld-%02lldT%02lld:%02lld:%02lld\n", y, mo, d, ss_h, ss_m, ss_s);
    printf("SN:%04lld-%02lld-%02lldT%02lld:%02lld\n", y, mo, d, sn_h, sn_m);
    printf("DL:%.2f\n", daylen);
    printf("ET:%.2f\n", EoT);
    printf("SD:%.2f\n", delta_deg);
    printf("SE:%.2f\n", elev);
    printf("JD:%.1f\n", jd);
}
