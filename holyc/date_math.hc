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

I64 IsLeap(I64 y) {
    if ((y % 4 == 0 && y % 100 != 0) || y % 400 == 0) { return 1; }
    return 0;
}

I64 DaysInMonth(I64 y, I64 m) {
    I64 d[] = {31,28,31,30,31,30,31,31,30,31,30,31};
    if (m == 2 && IsLeap(y)) return 29;
    return d[m-1];
}

I64 DaysInYear(I64 y) {
    if (IsLeap(y)) { return 366; }
    return 365;
}

I64 DateToDays(I64 y, I64 m, I64 d) {
    I64 total = 0, i;
    for (i = 1; i < y; i++) total += DaysInYear(i);
    for (i = 1; i < m; i++) total += DaysInMonth(y, i);
    return total + d - 1;
}

U0 DaysToDate(I64 days, I64 *y, I64 *m, I64 *d) {
    I64 yr = 1, mo, remaining = days;
    while (1) {
        I64 dy = DaysInYear(yr);
        if (remaining < dy) break;
        remaining -= dy;
        yr++;
    }
    *y = yr;
    for (mo = 1; mo <= 12; mo++) {
        I64 dm = DaysInMonth(yr, mo);
        if (remaining < dm) break;
        remaining -= dm;
    }
    *m = mo;
    *d = remaining + 1;
}

I64 ISOWeekNumber(I64 y, I64 m, I64 d) {
    I64 jan4_day = DateToDays(y, 1, 4);
    I64 current_day = DateToDays(y, m, d);
    I64 jan4_dow = (jan4_day + 1) % 7;
    if (jan4_dow == 0) jan4_dow = 7;
    I64 week1_start = jan4_day - (jan4_dow - 1);

    if (current_day < week1_start) {
        I64 prev_jan4 = DateToDays(y-1, 1, 4);
        I64 prev_dow = (prev_jan4 + 1) % 7;
        if (prev_dow == 0) prev_dow = 7;
        week1_start = prev_jan4 - (prev_dow - 1);
    }

    I64 week = (current_day - week1_start) / 7 + 1;

    I64 next_jan4 = DateToDays(y+1, 1, 4);
    I64 next_dow = (next_jan4 + 1) % 7;
    if (next_dow == 0) next_dow = 7;
    I64 last_week1_start = next_jan4 - (next_dow - 1);

    if (current_day >= last_week1_start) week = 1;

    return week;
}

U0 DiffDates(I64 y1, I64 m1, I64 d1, I64 y2, I64 m2, I64 d2) {
    I64 days1 = DateToDays(y1, m1, d1);
    I64 days2 = DateToDays(y2, m2, d2);
    I64 diff = days2 - days1;
    if (diff < 0) diff = -diff;

    I64 ey = y2, em = m2, ed = d2;
    I64 sy = y1, sm = m1, sd = d1;
    if (days2 < days1) {
        sy = y2; sm = m2; sd = d2;
        ey = y1; em = m1; ed = d1;
    }

    I64 years = ey - sy;
    I64 months = em - sm;
    I64 daydiff = ed - sd;

    if (daydiff < 0) {
        months--;
        I64 pm = em - 1;
        if (pm < 1) pm = 12;
        if (em > 1) { daydiff += DaysInMonth(ey, pm); }
        else { daydiff += DaysInMonth(ey-1, pm); }
    }
    if (months < 0) {
        years--;
        months += 12;
    }

    printf("DI:%lld\n", diff);
    printf("WK:%.1f\n", diff / 7.0);
    printf("MO:%lld\n", months + years * 12);
    printf("YR:%lld\n", years);
    printf("HR:%lld\n", diff * 24);
    printf("MI:%lld\n", diff * 1440);
    printf("SC:%lld\n", diff * 86400);
}

U0 AddDays(I64 y, I64 m, I64 d, I64 days) {
    I64 total = DateToDays(y, m, d) + days;
    I64 ny, nm, nd;
    DaysToDate(total, &ny, &nm, &nd);
    printf("DA:%04lld-%02lld-%02lld\n", ny, nm, nd);
}

U0 WeekOf(I64 y, I64 m, I64 d) {
    I64 week = ISOWeekNumber(y, m, d);
    I64 doy = DateToDays(y, m, d) - DateToDays(y, 1, 1) + 1;
    printf("WN:%02lld\n", week);
    printf("DY:%03lld\n", doy);
}

U0 Age(I64 by, I64 bm, I64 bd) {
    I64 rawtime;
    timeval tv;
    tm *ti;
    gettimeofday(&tv, 0);
    rawtime = tv.tv_sec;
    ti = localtime(&rawtime);
    I64 ty = ti->tm_year + 1900;
    I64 cur_mo = ti->tm_mon + 1;
    I64 td = ti->tm_mday;

    I64 years = ty - by;
    I64 months = cur_mo - bm;
    I64 days = td - bd;

    if (days < 0) {
        months--;
        I64 pm = cur_mo - 1;
        if (pm < 1) pm = 12;
        days += DaysInMonth(ty, pm);
    }
    if (months < 0) {
        years--;
        months += 12;
    }

    printf("AG:%lld years, %lld months, %lld days\n", years, months, days);
}

U0 Main(I64 argc, U8 **argv) {
    I64 y1, m1, d1, y2, m2, d2, pos, dayarg;

    if (argc < 2) {
        printf("Usage: date_math diff YYYY-MM-DD YYYY-MM-DD\n");
        printf("       date_math add YYYY-MM-DD days\n");
        printf("       date_math weekof YYYY-MM-DD\n");
        printf("       date_math age YYYY-MM-DD\n");
        return;
    }

    if (strcmp(argv[1], "diff") == 0 && argc >= 4) {
        pos = 0; y1 = ParseInt(argv[2], &pos); m1 = ParseInt(argv[2], &pos); d1 = ParseInt(argv[2], &pos);
        pos = 0; y2 = ParseInt(argv[3], &pos); m2 = ParseInt(argv[3], &pos); d2 = ParseInt(argv[3], &pos);
        DiffDates(y1, m1, d1, y2, m2, d2);
    } else if (strcmp(argv[1], "add") == 0 && argc >= 4) {
        pos = 0; y1 = ParseInt(argv[2], &pos); m1 = ParseInt(argv[2], &pos); d1 = ParseInt(argv[2], &pos);
        dayarg = strtoll(argv[3], NULL, 10);
        AddDays(y1, m1, d1, dayarg);
    } else if (strcmp(argv[1], "weekof") == 0 && argc >= 3) {
        pos = 0; y1 = ParseInt(argv[2], &pos); m1 = ParseInt(argv[2], &pos); d1 = ParseInt(argv[2], &pos);
        WeekOf(y1, m1, d1);
    } else if (strcmp(argv[1], "age") == 0 && argc >= 3) {
        pos = 0; y1 = ParseInt(argv[2], &pos); m1 = ParseInt(argv[2], &pos); d1 = ParseInt(argv[2], &pos);
        Age(y1, m1, d1);
    } else {
        printf("Unknown operation or missing arguments\n");
    }
}
