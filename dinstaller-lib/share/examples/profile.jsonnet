local dinstaller = import 'hw.libsonnet';
local findBiggestDisk(disks) =
  local sizedDisks = std.filter(function(d) std.objectHas(d, 'size'), disks);
  local sorted = std.sort(sizedDisks, function(x) x.size);
  sorted[0].logicalname;

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
      {
        name: findBiggestDisk(dinstaller.disks),
      },
    ],
  },
  scripts: [
    {
      type: 'post',
      url: 'https: //myscript.org/test.sh',
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
