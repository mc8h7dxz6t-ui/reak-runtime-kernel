#!/usr/bin/env bash
# CreditDuty rigorous validation (LandlordDuty moved to ~/landlord-duty)
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "== CreditDuty statutory =="
cd credit-duty
python3 -m pytest tests/statutory/ -v --tb=short

echo "== seed_demo =="
rm -rf exports
python3 scripts/seed_demo.py

echo "ALL TESTS PASSED"
