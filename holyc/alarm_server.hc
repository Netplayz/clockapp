#include "tos.HH"

I64 ParseInt(U8 *s, I64 *pos) {
    I64 val = 0;
    while (s[*pos] >= 0x30 && s[*pos] <= 0x39) {
        val = val * 10 + (s[*pos] - 0x30);
        (*pos)++;
    }
    return val;
}

U0 Main(I64 argc, U8 **argv) {
    U8 buf[4096];
    I32 fd;
    I64 i, alarm_h, alarm_m, now_h, now_m, nread, pos, line_start, line_end;
    U8 mode[16], label[128];
    I64 rawtime;
    timeval tv;
    tm *timeinfo;
    I64 match_found = 0;

    if (argc < 3 || strcmp(argv[1], "CHECK") != 0) {
        printf("Usage: alarm_server CHECK <file>\n");
        return;
    }

    gettimeofday(&tv, 0);
    rawtime = tv.tv_sec;
    timeinfo = localtime(&rawtime);
    now_h = timeinfo->tm_hour;
    now_m = timeinfo->tm_min;

    printf("CT:%02lld:%02lld\n", now_h, now_m);

    fd = open(argv[2], O_RDONLY);
    if (fd < 0) {
        printf("ALARM:ERROR_FILE_NOT_FOUND\n");
        return;
    }

    nread = read(fd, buf, 4095);
    close(fd);
    if (nread < 0) { printf("ALARM:ERROR_FILE_NOT_FOUND\n"); return; }
    buf[nread] = 0;

    line_start = 0;
    while (line_start < nread) {
        line_end = line_start;
        while (line_end < nread && buf[line_end] != 0x0A && buf[line_end] != 0x0D) line_end++;
        buf[line_end] = 0;

        U8 *line = buf + line_start;
        I64 len = strlen(line);
        if (len > 0 && line[0] != '#') {
            pos = 0;
            alarm_h = ParseInt(line, &pos);
            if (line[pos] == 0x3A) pos++;
            alarm_m = ParseInt(line, &pos);
            while (line[pos] == 0x20) pos++;

            i = 0;
            while (line[pos] != 0x20 && line[pos] != 0 && i < 15) {
                mode[i++] = line[pos++];
            }
            mode[i] = 0;
            while (line[pos] == 0x20) pos++;

            i = 0;
            while (line[pos] != 0 && i < 127) {
                label[i++] = line[pos++];
            }
            label[i] = 0;

            if (alarm_h == now_h && alarm_m == now_m) {
                printf("ALARM:%s\n", label);
                match_found = 1;
            }
        }

        line_start = line_end + 1;
        while (line_start < nread && (buf[line_start] == 0x0A || buf[line_start] == 0x0D)) line_start++;
    }

    if (!match_found)
        printf("ALARM:NONE\n");
}
