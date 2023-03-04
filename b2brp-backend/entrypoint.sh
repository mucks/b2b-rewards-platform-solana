#!/bin/sh
set -eu

npx prisma db push
./b2brp-backend