#!/bin/bash

#This is a half working failed attempt to solve the task using xsv

{
	echo "country,tz";
	{
		echo country;
		xsv select country,tz geolite2-city-ipv4-num.csv | \
			tail -n+2 | \
			sort | \
			uniq | \
			cut -d',' -f1 | \
			uniq -c | \
			sort -n  | \
			sed 's/^ *//' | \
			grep '^1 ' | \
			cut -d' ' -f2;
	} | xsv join --left country /dev/stdin country geolite2-city-ipv4-num.csv | \
		xsv select country,tz | \
		tail -n+2 | \
		sort | \
		uniq;
} | xsv join --full country /dev/stdin country dbip-city-ipv4-num.csv
