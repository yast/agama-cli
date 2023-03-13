#!/usr/bin/sh
set -ex

if [ -z "$1" ]
then
  url=`cat /proc/cmdline | awk -F 'dinst.auto=' '{sub(/ .*$/, "", $2); print $2}'`
else
  url="$1"
fi

echo "Using the profile at $url"

tmpdir=$(mktemp --directory --suffix "-dinstaller")
echo working on $tmpdir
/usr/bin/dinstaller-cli profile download $url > ${tmpdir}/profile.jsonnet
/usr/bin/dinstaller-cli profile evaluate ${tmpdir}/profile.jsonnet > ${tmpdir}/profile.json
/usr/bin/dinstaller-cli profile validate ${tmpdir}/profile.json
/usr/bin/dinstaller-cli config load ${tmpdir}/profile.json
rm -r $tmpdir
# not implemented yet:
# /usr/bin/dinstaller-cli watch
