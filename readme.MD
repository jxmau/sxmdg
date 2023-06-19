# sqlx-md-gen

sqlx-md-gen is a small piece of software generating docs for sqlx-cli's migrations folder to keep track of the changes made.

## Requirements

* None so far

## Commands

### generate

Generate a markdone file named `database.MD` that will list every migrations files sorted by tables.

`sqlx-md-gen generate <path>`

flag/variables | Option | Notes
--- | --- | --- |
path | Yes | Give the path of the migrations directory

### sheet

Will add a blank form for the Metadata at the beginning of the file. It will only work if it's an SQL file and if the software decides that a form hasn't been added before..

`sqlx-md-gen sheet <path>`

flag/variables | Option | Notes
--- | --- | --- |
path | No | Give the path of the SQL file
