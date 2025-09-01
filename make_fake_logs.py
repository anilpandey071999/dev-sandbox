# /// script
# requires-python = ">=3.8"
# dependencies = [
#   "Faker>=26"
# ]
# ///
#!/usr/bin/env python3
# make_fake_logs.py
# Generate fake logs: Apache CLF/Combined, RFC 5424 syslog, JSON logs, and simple app logs.

# command for generating the logs 
#  pipx run --path ./make_fake_logs.py --out ./fake-logs --files 5

import argparse
import json
import os
import random
import string
from datetime import datetime, timedelta, timezone
from pathlib import Path
from uuid import uuid4

from faker import Faker

METHODS = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"]
LEVELS = ["DEBUG", "INFO", "WARN", "ERROR"]
HTTP_STATUSES = [200, 200, 200, 201, 204, 301, 302, 304, 400, 401, 403, 404, 429, 500, 502, 503, 504]
APPS = ["authsvc", "ordersvc", "paymentsvc", "web", "worker", "cron"]
FACILITIES = list(range(0, 24))  # RFC 5424 facilities 0..23
SEVERITIES = list(range(0, 8))   # RFC 5424 severities 0..7

def rand_dt_utc(within_days: int) -> datetime:
    now = datetime.now(timezone.utc)
    delta = timedelta(seconds=random.randint(0, max(1, within_days) * 86400))
    return now - delta

def ts_apache(dt: datetime) -> str:
    # CLF timestamp: [dd/Mon/yyyy:HH:MM:SS +0000]
    return dt.strftime("%d/%b/%Y:%H:%M:%S %z")

def iso_millis(dt: datetime) -> str:
    # ISO-8601 with milliseconds and Z for UTC
    return dt.isoformat(timespec="milliseconds").replace("+00:00", "Z")

def random_path(fake: Faker) -> str:
    base = fake.uri_path()
    # add a few fixed prefixes to simulate API layout
    prefixes = ["/", "/api", "/v1", "/v2", "/static", "/content"]
    prefix = random.choice(prefixes)
    path = prefix.rstrip("/") + "/" + base.lstrip("/")
    # sometimes add a file-like suffix
    if random.random() < 0.25:
        path = path.rstrip("/") + random.choice([".html", ".css", ".js", ".png", ".jpg", ".svg"])
    return path

def random_query(fake: Faker) -> str:
    if random.random() < 0.6:
        return ""
    params = []
    for _ in range(random.randint(1, 3)):
        k = fake.word().lower()
        v = fake.word().lower()
        params.append(f"{k}={v}")
    return "?" + "&".join(params)

def apache_line(fake: Faker, dt: datetime, style: str = "common") -> str:
    ip = fake.ipv4_public()
    ident = "-"  # rarely used
    user = fake.user_name() if random.random() < 0.15 else "-"
    path = random_path(fake) + random_query(fake)
    method = random.choice(METHODS)
    proto = random.choice(["HTTP/1.1", "HTTP/2"])
    status = random.choice(HTTP_STATUSES)
    size = 0 if status in (204, 304) else random.randint(128, 50000)

    core = f'{ip} {ident} {user} [{ts_apache(dt)}] "{method} {path} {proto}" {status} {size}'
    if style == "combined":
        referer = "-" if random.random() < 0.4 else fake.url()
        try:
            ua = fake.user_agent()
        except Exception:
            ua = "Mozilla/5.0"
        return f'{core} "{referer}" "{ua}"'
    return core

def syslog_rfc5424_line(fake: Faker, dt: datetime) -> str:
    facility = random.choice(FACILITIES)
    severity = random.choice(SEVERITIES)
    pri = facility * 8 + severity  # PRI = Facility*8 + Severity
    hostname = fake.hostname()
    app = random.choice(APPS)
    pid = random.randint(100, 65535)
    msgid = f"ID{random.randint(10,99)}"
    # structured data placeholder '-'
    msg = fake.sentence(nb_words=random.randint(5, 12)).replace("\n", " ")
    return f"<{pri}>1 {iso_millis(dt)} {hostname} {app} {pid} {msgid} - {msg}"

def json_line(fake: Faker, dt: datetime) -> str:
    method = random.choice(METHODS)
    path = random_path(fake)
    status = random.choice(HTTP_STATUSES)
    size = 0 if status in (204, 304) else random.randint(64, 90000)
    referer = None if random.random() < 0.5 else fake.url()
    try:
        ua = fake.user_agent()
    except Exception:
        ua = "Mozilla/5.0"
    doc = {
        "ts": iso_millis(dt),
        "level": random.choice(LEVELS),
        "msg": fake.sentence(nb_words=random.randint(6, 14)),
        "method": method,
        "path": path,
        "status": status,
        "bytes": size,
        "ip": fake.ipv4_public(),
        "user_agent": ua,
        "referer": referer,
        "request_id": uuid4().hex,
        "duration_ms": round(random.uniform(0.3, 800.0), 2),
    }
    return json.dumps(doc, ensure_ascii=False)

def app_kv_line(fake: Faker, dt: datetime) -> str:
    level = random.choice(LEVELS)
    rid = uuid4().hex[:12]
    user = fake.user_name()
    action = random.choice(["login", "logout", "purchase", "view", "update", "delete"])
    ok = random.random() > 0.1
    return (
        f"{iso_millis(dt)} level={level} req_id={rid} user={user} "
        f"action={action} ok={str(ok).lower()} message='{fake.sentence(nb_words=8)}'"
    )

def generate_file(fake: Faker, fmt: str, out_file: Path, n_lines: int, days: int, apache_style: str):
    with out_file.open("w", encoding="utf-8") as f:
        for _ in range(n_lines):
            dt = rand_dt_utc(days)
            if fmt == "apache":
                line = apache_line(fake, dt, apache_style)
            elif fmt == "syslog":
                line = syslog_rfc5424_line(fake, dt)
            elif fmt == "json":
                line = json_line(fake, dt)
            elif fmt == "app":
                line = app_kv_line(fake, dt)
            else:
                raise ValueError(f"Unknown format: {fmt}")
            f.write(line + "\n")

def main():
    parser = argparse.ArgumentParser(description="Generate fake log files for testing.")
    parser.add_argument("--out", type=Path, default=Path("./fake-logs"), help="Output root directory")
    parser.add_argument("--files", type=int, default=5, help="Number of files per format")
    parser.add_argument("--min-lines", type=int, default=500, help="Minimum lines per file")
    parser.add_argument("--max-lines", type=int, default=2000, help="Maximum lines per file")
    parser.add_argument("--formats", type=str, default="apache,syslog,json,app",
                        help="Comma-separated formats: apache,syslog,json,app")
    parser.add_argument("--days", type=int, default=7, help="Spread timestamps within last N days")
    parser.add_argument("--apache-style", type=str, default="common", choices=["common", "combined"],
                        help="Apache log style: common (CLF) or combined")
    parser.add_argument("--seed", type=int, default=None, help="Random seed for reproducibility")

    args = parser.parse_args()
    if args.seed is not None:
        random.seed(args.seed)

    fake = Faker()
    # Optional: localize if desired, e.g., Faker('en_IN') for Indian locale data
    # fake = Faker('en_IN')

    formats = [s.strip().lower() for s in args.formats.split(",") if s.strip()]
    args.out.mkdir(parents=True, exist_ok=True)

    for fmt in formats:
        sub = args.out / fmt
        sub.mkdir(parents=True, exist_ok=True)
        for i in range(1, args.files + 1):
            n = random.randint(args.min_lines, max(args.min_lines, args.max_lines))
            name = f"{fmt}_log_{i:04d}.log" if fmt != "json" else f"{fmt}_log_{i:04d}.jsonl"
            out_file = sub / name
            generate_file(fake, fmt, out_file, n, args.days, args.apache_style)

    print(f"Done. Logs under: {args.out.resolve()}")

if __name__ == "__main__":
    main()
