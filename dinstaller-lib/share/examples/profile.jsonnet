local dinstaller = import 'dinstaller.libsonnet';
local findBiggestDisk(disks) =
  local sorted = std.sort(disks, function(x) x.size);
  sorted[0].name;

{
  software: {
    product: 'ALP',
  },
  user: {
    fullName: 'Jane Doe',
    userName: 'jane.doe',
    password: '123456',
  },
  // look ma, there are comments!
  localization: {
    language: 'en_US',
    keyboard: 'en_US',
  },
  storage: {
    devices: [
      { name: findBiggestDisk(dinstaller.disks) },
    ],
  },
  scripts: [
    {
      type: 'post',
      url: 'https://myscript.org/test.sh',
    },
    {
      type: 'pre',
      source: |||
        #!/bin/bash

        echo hello
      |||,
    },
  ],
}
