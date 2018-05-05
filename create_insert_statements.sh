#!/bin/bash

lines=`cat link_titles.txt`

while read p; do
  echo "INSERT INTO links_processed (title) values ('$p');"
done <link_titles.txt


# echo "$lines"