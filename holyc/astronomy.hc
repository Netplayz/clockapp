#include "tos.HH"

I64 ParseInt(U8 *s, I64 *pos) {
    I64 val = 0;
    while (s[*pos] == 0x20 || s[*pos] == 0x2D) { (*pos)++; }
    while (s[*pos] >= 0x30 && s[*pos] <= 0x39) {
        val = val * 10 + (s[*pos] - 0x30);
        (*pos)++;
    }
    return val;
}

F64 CalcJD(I64 y, I64 m, I64 d) {
    I64 a, b;
    if (m <= 2) { y--; m += 12; }
    a = y / 100;
    b = 2 - a + a / 4;
    return trunc(365.25 * (y + 4716)) + trunc(30.6001 * (m + 1)) + d + b - 2453069.5;
}

F64 CalcMoonPhase(I64 y, I64 m, I64 d) {
    F64 jd = CalcJD(y, m, d) + 2451545.0;
    F64 days_since = jd - 2451550.09765;
    F64 phase = (days_since / 29.53058867);
    F64 ip;
    modf(phase, &ip);
    phase = phase - ip;
    if (phase < 0) phase += 1.0;
    return phase;
}

U0 GetMoonPhaseName(F64 phase, U8 *buf) {
    if (phase < 0.0625 || phase >= 0.9375)
        StrCpy(buf, "New Moon");
    else if (phase < 0.1875)
        StrCpy(buf, "Waxing Crescent");
    else if (phase < 0.3125)
        StrCpy(buf, "First Quarter");
    else if (phase < 0.4375)
        StrCpy(buf, "Waxing Gibbous");
    else if (phase < 0.5625)
        StrCpy(buf, "Full Moon");
    else if (phase < 0.6875)
        StrCpy(buf, "Waning Gibbous");
    else if (phase < 0.8125)
        StrCpy(buf, "Last Quarter");
    else
        StrCpy(buf, "Waning Crescent");
}

F64 CalcSolarLongitude(I64 y, I64 m, I64 d) {
    F64 jd = CalcJD(y, m, d);
    F64 n = jd;
    F64 L = 280.460 + 0.9856474 * n;
    F64 g = 357.528 + 0.9856003 * n;
    F64 lambda = L + 1.915 * sin(g * 3.14159 / 180.0) + 0.020 * sin(2 * g * 3.14159 / 180.0);
    lambda = fmod(lambda, 360.0);
    if (lambda < 0) lambda += 360.0;
    return lambda;
}

U0 GetSeason(F64 solar_lon, U8 *buf) {
    if (solar_lon >= 0 && solar_lon < 90)
        StrCpy(buf, "Spring");
    else if (solar_lon >= 90 && solar_lon < 180)
        StrCpy(buf, "Summer");
    else if (solar_lon >= 180 && solar_lon < 270)
        StrCpy(buf, "Autumn");
    else
        StrCpy(buf, "Winter");
}

U0 Main(I64 argc, U8 **argv) {
    I64 y, m, d, pos;
    F64 phase, jd, solar_lon, days_since, ip;
    U8 phase_name[32], season[16];

    if (argc < 2) { printf("Usage: astronomy YYYY-MM-DD\n"); return; }

    pos = 0;
    y = ParseInt(argv[1], &pos);
    m = ParseInt(argv[1], &pos);
    d = ParseInt(argv[1], &pos);

    jd = CalcJD(y, m, d) + 2451545.0;
    phase = CalcMoonPhase(y, m, d);
    GetMoonPhaseName(phase, phase_name);
    solar_lon = CalcSolarLongitude(y, m, d);
    GetSeason(solar_lon, season);

    F64 new_moon_jd = 2451550.09765;
    F64 synodic = 29.53058867;
    F64 days_since_new = jd - new_moon_jd;
    modf(days_since_new / synodic, &ip);
    days_since_new = days_since_new - ip * synodic;
    if (days_since_new < 0) days_since_new += synodic;

    printf("JD:%.1f\n", jd);
    printf("MP:%.2f\n", phase);
    printf("MN:%s\n", phase_name);
    printf("SN:%s\n", season);
    printf("DN:%.1f\n", days_since_new);
    printf("NN:%.1f\n", synodic - days_since_new);
}
