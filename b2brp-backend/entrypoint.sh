#!/bin/sh
set -eu

npx prisma db push
./otalp_backend