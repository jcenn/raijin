## TODO
- purge command
- remove command
- better spacing when printing all entries
- custom settings (maybe config.yml)


Required some bash tinkering but works

~/.bashrc
> alias rjn='source /home/jcen/custom-scripts/raijin.sh'

~/custom-scripts/raijin.sh
"""
#!/usr/bin/env bash

OUTPUT=$(/home/jcen/personal/rust/raijin/target/debug/raijin $1 $2 $3)

if [[ $OUTPUT = '/'* ]]
then
	cd $OUTPUT
else
	echo "$OUTPUT"
fi
"""
