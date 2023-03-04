#!/bin/sh

npx prisma db push
diesel print-schema > src/schema.rs

filename="src/db/models.rs"

diesel_ext -m -t \
	--derive "Serialize,Deserialize,Queryable" \
	-I "crate::schema::*" \
	-I "diesel::{Queryable, Identifiable}" \
	-I "serde::{Serialize,Deserialize}" > $filename

search='"'
replace=''

sed -i "s/$search/$replace/" $filename
sed -i "s/$search/$replace/" $filename