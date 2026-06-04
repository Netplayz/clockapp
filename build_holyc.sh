#!/bin/bash
set -e
export PATH="$HOME/.local/bin:$PATH"
cd "$(dirname "$0")/holyc"
echo "Compiling HolyC programs..."
hcc solar_calc.hc -o solar_calc 2>&1
hcc astronomy.hc -o astronomy 2>&1
hcc alarm_server.hc -o alarm_server 2>&1
hcc date_math.hc -o date_math 2>&1
echo "All HolyC programs compiled successfully."
