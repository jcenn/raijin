## TODO
- [x] purge command
- [x] remove command
- [ ] modifying existing entries
- [ ] better spacing when printing all entries
- [ ] custom settings (maybe config.yml)
- [x] fix printing empty line when commands don't output anything

Required some bash tinkering but finally works

> ~/.bashrc
```bash
alias rjn='source /home/jcen/custom-scripts/raijin.sh'
```

> ~/custom-scripts/raijin.sh
```bash
#!/usr/bin/env bash

OUTPUT=$(/home/jcen/personal/rust/raijin/target/debug/raijin $1 $2 $3)

if [[ $OUTPUT = '/'* ]]; then
	cd $OUTPUT
elif [[ -n "$OUTPUT" ]]; then
	echo "$OUTPUT"
fi
```
