# rate-swallow
Used to sample stdin.

For example, to the left of the pip we print a count out and rate-swallow will limit the output to one line per second (or as configured about args) and disregard the rest.

```
$ (x=0; while true; do x=$((x+1)); echo $x; done;) | ./target/release/rate-swallow
1
150720
302176
454391
606746
759397
911644
1063461
1215517
1367239
1518868
```
