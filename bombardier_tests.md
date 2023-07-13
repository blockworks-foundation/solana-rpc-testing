# Validator RPC tests with Bombardier

All test are done on the solana-test-validator.

## Test1: getBalance

### HTTP request

```
curl http://192.168.88.12:8899 -X POST -H "Content-Type: application/json" -d '
  {
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getBalance",
    "params": [
      "4EVAJ81v9fLjz2wp44SZmxgTbvYB8dBRZ6s1SAqP99eZ"
    ]
  }
'
```

### Bombardier cmd

```
~/go/bin/bombardier -m POST http://validator_ip:8899 -c 125 -n 1000000 -l -H "Content-Type: application/json" -b '{"jsonrpc": "2.0","id": 1,"method": "getBalance","params": ["4EVAJ81v9fLjz2wp44SZmxgTbvYB8dBRZ6s1SAqP99eZ"]}'
```

-c indicate the number of simultaneous connections

-n the number of request send during the test.


### Master branch

125 concurrent connections

```
└─> ~/go/bin/bombardier -m POST http://192.168.88.12:8899 -c 125 -n 1000000 -l -H "Content-Type: application/json" -b '{"jsonrpc": "2.0","id": 1,"method": "getBalance","params": ["4EVAJ81v9fLjz2wp44SZmxgTbvYB8dBRZ6s1SAqP99eZ"]}'
Bombarding http://192.168.88.12:8899 with 1000000 request(s) using 125 connection(s)
 1000000 / 1000000 [=======================================================================================================================] 100.00% 64877/s 15s
Done!
Statistics        Avg      Stdev        Max
  Reqs/sec     65325.94    3999.18   70076.82
  Latency        1.91ms   124.33us    12.73ms
  Latency Distribution
     50%     1.88ms
     75%     1.98ms
     90%     2.01ms
     95%     2.04ms
     99%     2.17ms
  HTTP codes:
    1xx - 0, 2xx - 1000000, 3xx - 0, 4xx - 0, 5xx - 0
    others - 0
  Throughput:    28.28MB/s

```

2000 concurrent connections

```
└─> ~/go/bin/bombardier -m POST http://192.168.88.12:8899 -c 2000 -n 1000000 -l -H "Content-Type: application/json" -b '{"jsonrpc": "2.0","id": 1,"method": "getBalance","params": ["4EVAJ81v9fLjz2wp44SZmxgTbvYB8dBRZ6s1SAqP99eZ"]}'
Bombarding http://192.168.88.12:8899 with 1000000 request(s) using 2000 connection(s)
 1000000 / 1000000 [=======================================================================================================================] 100.00% 63872/s 15s
Done!
Statistics        Avg      Stdev        Max
  Reqs/sec     64491.53    4967.52   72697.52
  Latency       31.01ms   187.19ms      1.73s
  Latency Distribution
     50%     3.00ms
     75%     3.55ms
     90%     3.93ms
     95%     4.21ms
     99%      1.40s
  HTTP codes:
    1xx - 0, 2xx - 1000000, 3xx - 0, 4xx - 0, 5xx - 0
    others - 0
  Throughput:    27.77MB/s

```

Time out limit: 2800

### tokio_jsonrpsee branch

125 concurrent connections

```
└─> ~/go/bin/bombardier -m POST http://192.168.88.12:8899 -c 125 -n 1000000 -l -H "Content-Type: application/json" -b '{"jsonrpc": "2.0","id": 1,"method": "getBalance","params": ["4EVAJ81v9fLjz2wp44SZmxgTbvYB8dBRZ6s1SAqP99eZ"]}'
Bombarding http://192.168.88.12:8899 with 1000000 request(s) using 125 connection(s)
 1000000 / 1000000 [=======================================================================================================================] 100.00% 75683/s 13s
Done!
Statistics        Avg      Stdev        Max
  Reqs/sec     76484.09    7319.42   91790.26
  Latency        1.63ms    80.77us     7.92ms
  Latency Distribution
     50%     1.68ms
     75%     1.72ms
     90%     1.75ms
     95%     1.77ms
     99%     1.89ms
  HTTP codes:
    1xx - 0, 2xx - 1000000, 3xx - 0, 4xx - 0, 5xx - 0
    others - 0
  Throughput:    41.86MB/s

```

2000 concurrent connections

```
└─> ~/go/bin/bombardier -m POST http://192.168.88.12:8899 -c 2000 -n 1000000 -l -H "Content-Type: application/json" -b '{"jsonrpc": "2.0","id": 1,"method": "getBalance","params": ["4EVAJ81v9fLjz2wp44SZmxgTbvYB8dBRZ6s1SAqP99eZ"]}'
Bombarding http://192.168.88.12:8899 with 1000000 request(s) using 2000 connection(s)
 1000000 / 1000000 [=======================================================================================================================] 100.00% 69928/s 14s
Done!
Statistics        Avg      Stdev        Max
  Reqs/sec     70845.62    5685.06   79450.53
  Latency       28.27ms   187.93ms      1.76s
  Latency Distribution
     50%     2.71ms
     75%     3.23ms
     90%     3.56ms
     95%     3.75ms
     99%      1.50s
  HTTP codes:
    1xx - 0, 2xx - 1000000, 3xx - 0, 4xx - 0, 5xx - 0
    others - 0
  Throughput:    38.47MB/s

```

Time out limit: 2500

### jsonrpc_actix branch

125 concurrent connections

```
└─> ~/go/bin/bombardier -m POST http://192.168.88.12:8899 -c 125 -n 1000000 -l -H "Content-Type: application/json" -b '{"jsonrpc": "2.0","id": 1,"method": "getBalance","params": ["4EVAJ81v9fLjz2wp44SZmxgTbvYB8dBRZ6s1SAqP99eZ"]}'
Bombarding http://192.168.88.12:8899 with 1000000 request(s) using 125 connection(s)
 1000000 / 1000000 [=====================================================================================================================================] 100.00% 184913/s 5s
Done!
Statistics        Avg      Stdev        Max
  Reqs/sec    185880.61   26455.56  259388.41
  Latency      670.18us   362.49us    17.54ms
  Latency Distribution
     50%   624.00us
     75%   809.00us
     90%     0.89ms
     95%     0.93ms
     99%     1.50ms
  HTTP codes:
    1xx - 0, 2xx - 1000000, 3xx - 0, 4xx - 0, 5xx - 0
    others - 0
  Throughput:    77.81MB/s

```

2000 concurrent connections

```
└─> ~/go/bin/bombardier -m POST http://192.168.88.12:8899 -c 2000 -n 1000000 -l -H "Content-Type: application/json" -b '{"jsonrpc": "2.0","id": 1,"method": "getBalance","params": ["4EVAJ81v9fLjz2wp44SZmxgTbvYB8dBRZ6s1SAqP99eZ"]}'
Bombarding http://192.168.88.12:8899 with 1000000 request(s) using 2000 connection(s)
 1000000 / 1000000 [=====================================================================================================================================] 100.00% 198302/s 5s
Done!
Statistics        Avg      Stdev        Max
  Reqs/sec    202315.55   32308.67  269518.07
  Latency        9.89ms     4.58ms   223.11ms
  Latency Distribution
     50%     8.14ms
     75%    12.03ms
     90%    13.40ms
     95%    13.94ms
     99%    24.31ms
  HTTP codes:
    1xx - 0, 2xx - 1000000, 3xx - 0, 4xx - 0, 5xx - 0
    others - 0
  Throughput:    83.66MB/s

```

Time out limit: No time out at 10 000


## Test2: getBlocks

### HTTP request

```
curl http://192.168.88.12:8899 -X POST -H "Content-Type: application/json" -d '
  {
    "jsonrpc": "2.0", "id": 1,
    "method": "getBlocks",
    "params": [
      0, 1000000
    ]
  }
'
```

### Bombardier cmd

```
~/go/bin/bombardier -m POST http://192.168.88.12:8899 -c 500 --duration 10s -l -H "Content-Type: application/json" -b '{"jsonrpc": "2.0", "id": 1,"method": "getBlocks","params": [0, 1000000]}'

```

-c indicate the number of simultaneous connections

--duration duration of the test.


### Master branch

500 concurrent connections

```
└─> ~/go/bin/bombardier -m POST http://192.168.88.12:8899 -c 500 --duration 10s -l -H "Content-Type: application/json" -b '{"jsonrpc": "2.0", "id": 1,"method": "getBlocks","params": [0, 1000000]}'
Bombarding http://192.168.88.12:8899 for 10s using 500 connection(s)
[========================================================================================================================================================================] 10s
Done!
Statistics        Avg      Stdev        Max
  Reqs/sec      3315.07     555.19    7302.11
  Latency      149.58ms   194.00ms      2.03s
  Latency Distribution
     50%    75.65ms
     75%    76.39ms
     90%    77.82ms
     95%   135.63ms
     99%      2.00s
  HTTP codes:
    1xx - 0, 2xx - 32365, 3xx - 0, 4xx - 0, 5xx - 0
    others - 1293
  Errors:
       timeout - 1293
  Throughput:    27.43MB/s

```

### tokio_jsonrpsee branch

500 concurrent connections

```
└─> ~/go/bin/bombardier -m POST http://192.168.88.12:8899 -c 500 --duration 10s -l -H "Content-Type: application/json" -b '{"jsonrpc": "2.0", "id": 1,"method": "getBlocks","params": [0, 1000000]}'
Bombarding http://192.168.88.12:8899 for 10s using 500 connection(s)
[========================================================================================================================================================================] 10s
Done!
Statistics        Avg      Stdev        Max
  Reqs/sec      4754.51     622.91    9453.57
  Latency      104.57ms   149.29ms      2.00s
  Latency Distribution
     50%    52.15ms
     75%    52.81ms
     90%    53.56ms
     95%    81.62ms
     99%      2.00s
  HTTP codes:
    1xx - 0, 2xx - 46680, 3xx - 0, 4xx - 0, 5xx - 0
    others - 1228
  Errors:
       timeout - 1228
  Throughput:    27.39MB/s

```

### jsonrpc_actix branch

500 concurrent connections

```
└─> ~/go/bin/bombardier -m POST http://192.168.88.12:8899 -c 500 --duration 10s -l -H "Content-Type: application/json" -b '{"jsonrpc": "2.0", "id": 1,"method": "getBlocks","params": [0, 1000000]}'
Bombarding http://192.168.88.12:8899 for 10s using 500 connection(s)
[========================================================================================================================================================================] 10s
Done!
Statistics        Avg      Stdev        Max
  Reqs/sec     24108.05    2054.19   27708.46
  Latency       20.72ms    13.15ms   567.71ms
  Latency Distribution
     50%    16.29ms
     75%    20.52ms
     90%    26.61ms
     95%    37.33ms
     99%    95.14ms
  HTTP codes:
    1xx - 0, 2xx - 241221, 3xx - 0, 4xx - 0, 5xx - 0
    others - 0
  Throughput:   111.27MB/s

```

## Test3: getBlock

### HTTP request

```
curl http://192.168.88.12:8899 -X POST -H "Content-Type: application/json" -d '
  {
    "jsonrpc": "2.0","id":1,
    "method":"getBlock",
    "params": [
      184843,
      {
        "encoding": "json",
        "maxSupportedTransactionVersion":0,
        "transactionDetails":"full",
        "rewards":false
      }
    ]
  }
'
```

### Bombardier cmd

```
~/go/bin/bombardier -m POST http://192.168.88.12:8899 -c 100 --duration 10s -l -H "Content-Type: application/json" -b '{"jsonrpc": "2.0","id":1,"method":"getBlock","params":[184843,{"encoding": "json","maxSupportedTransactionVersion":0,"transactionDetails":"full","rewards":false}]}'

```

-c indicate the number of simultaneous connections

--duration duration of the test.


### Master branch

```
└─> ~/go/bin/bombardier -m POST http://192.168.88.12:8899 -c 200 --duration 10s -l -H "Content-Type: application/json" -b '{"jsonrpc": "2.0","id":1,"method":"getBlock","params":[187390,{"encoding": "json","maxSupportedTransactionVersion":0,"transactionDetails":"full","rewards":false}]}'
Bombarding http://192.168.88.12:8899 for 10s using 200 connection(s)
[========================================================================================================================================================================] 10s
Done!
Statistics        Avg      Stdev        Max
  Reqs/sec      3882.70     330.33    4455.36
  Latency       51.38ms     4.75ms   166.34ms
  Latency Distribution
     50%    50.80ms
     75%    52.45ms
     90%    53.91ms
     95%    54.65ms
     99%    58.50ms
  HTTP codes:
    1xx - 0, 2xx - 39022, 3xx - 0, 4xx - 0, 5xx - 0
    others - 0
  Throughput:     7.42MB/s
```

Time out limit: 300

### tokio_jsonrpsee branch

200 concurrent connections

```
└─> ~/go/bin/bombardier -m POST http://192.168.88.12:8899 -c 200 --duration 10s -l -H "Content-Type: application/json" -b '{"jsonrpc": "2.0","id":1,"method":"getBlock","params":[187390,{"encoding": "json","maxSupportedTransactionVersion":0,"transactionDetails":"full","rewards":false}]}'
Bombarding http://192.168.88.12:8899 for 10s using 200 connection(s)
[========================================================================================================================================================================] 10s
Done!
Statistics        Avg      Stdev        Max
  Reqs/sec      4263.59     380.01    5064.36
  Latency       46.79ms     4.28ms   149.55ms
  Latency Distribution
     50%    45.86ms
     75%    47.72ms
     90%    49.81ms
     95%    51.56ms
     99%    57.42ms
  HTTP codes:
    1xx - 0, 2xx - 42825, 3xx - 0, 4xx - 0, 5xx - 0
    others - 0
  Throughput:     8.64MB/s


```

Time out limit: 300

### jsonrpc_actix branch

200 concurrent connections

```
└─> ~/go/bin/bombardier -m POST http://192.168.88.12:8899 -c 200 --duration 10s -l -H "Content-Type: application/json" -b '{"jsonrpc": "2.0","id":1,"method":"getBlock","params":[187390,{"encoding": "json","maxSupportedTransactionVersion":0,"transactionDetails":"full","rewards":false}]}'
Bombarding http://192.168.88.12:8899 for 10s using 200 connection(s)
[========================================================================================================================================================================] 10s
Done!
Statistics        Avg      Stdev        Max
  Reqs/sec     14901.73     971.98   16324.93
  Latency       13.41ms     2.03ms    50.24ms
  Latency Distribution
     50%    14.19ms
     75%    15.71ms
     90%    17.36ms
     95%    18.41ms
     99%    20.71ms
  HTTP codes:
    1xx - 0, 2xx - 149207, 3xx - 0, 4xx - 0, 5xx - 0
    others - 0
  Throughput:    28.25MB/s

```

Time out limit: No timeout at 10000

## Test4: getSlot

### HTTP request

```
curl http://192.168.88.12:8899 -X POST -H "Content-Type: application/json" -d '
  {"jsonrpc":"2.0","id":1, "method":"getSlot"}
'
```

### Bombardier cmd

```
~/go/bin/bombardier -m POST http://192.168.88.12:8899 -c 100 --duration 10s -l -H "Content-Type: application/json" -b '{"jsonrpc":"2.0","id":1, "method":"getSlot"}'

```

-c indicate the number of simultaneous connections

--duration duration of the test.


### Master branch

500 concurrent connections

```
└─> ~/go/bin/bombardier -m POST http://192.168.88.12:8899 -c 500 --duration 10s -l -H "Content-Type: application/json" -b '{"jsonrpc":"2.0","id":1, "method":"getSlot"}'
Bombarding http://192.168.88.12:8899 for 10s using 500 connection(s)
[========================================================================================================================================================================] 10s
Done!
Statistics        Avg      Stdev        Max
  Reqs/sec     76056.20    5423.20   86208.69
  Latency        6.58ms    27.37ms   262.22ms
  Latency Distribution
     50%     2.54ms
     75%     2.97ms
     90%     3.29ms
     95%     3.43ms
     99%   207.17ms
  HTTP codes:
    1xx - 0, 2xx - 758937, 3xx - 0, 4xx - 0, 5xx - 0
    others - 0
  Throughput:    23.65MB/s
```

### tokio_jsonrpsee branch

500 concurrent connections

```
└─> ~/go/bin/bombardier -m POST http://192.168.88.12:8899 -c 500 --duration 10s -l -H "Content-Type: application/json" -b '{"jsonrpc":"2.0","id":1, "method":"getSlot"}'
Bombarding http://192.168.88.12:8899 for 10s using 500 connection(s)
[========================================================================================================================================================================] 10s
Done!
Statistics        Avg      Stdev        Max
  Reqs/sec     79002.32    7471.10   94644.33
  Latency        6.33ms    26.41ms   259.82ms
  Latency Distribution
     50%     2.44ms
     75%     2.87ms
     90%     3.22ms
     95%     3.40ms
     99%   198.62ms
  HTTP codes:
    1xx - 0, 2xx - 788621, 3xx - 0, 4xx - 0, 5xx - 0
    others - 0
  Throughput:    24.58MB/s

```


### jsonrpc_actix branch

500 concurrent connections

```
└─> ~/go/bin/bombardier -m POST http://192.168.88.12:8899 -c 500 --duration 10s -l -H "Content-Type: application/json" -b '{"jsonrpc":"2.0","id":1, "method":"getSlot"}'
Bombarding http://192.168.88.12:8899 for 10s using 500 connection(s)
[========================================================================================================================================================================] 10s
Done!
Statistics        Avg      Stdev        Max
  Reqs/sec    291864.09   28315.98  336660.32
  Latency        1.70ms   412.45us    44.95ms
  Latency Distribution
     50%     1.67ms
     75%     1.81ms
     90%     2.10ms
     95%     2.47ms
     99%     3.22ms
  HTTP codes:
    1xx - 0, 2xx - 2915955, 3xx - 0, 4xx - 0, 5xx - 0
    others - 0
  Throughput:    86.12MB/s

```



## Conclusion

The tokio_jsonrpsee show a little improvement for both request compared to master branch.

For the getBlocks the jsonrpc_actix show an important improvement of the performance. I've to find the reason because I don't think it's only related to the json serialisation and HTTP request handling. This improvement mus tbe confirmed with the getBalance request when it will be available.

## Actions

 - correct getBalance on jsonrpc_actix
 - Do the getBalance test for jsonrpc_actix branch
 - See the reason of the performance difference between jsonrpc_actix branch and the others.


 GetBLockHash GetSlot.

 curl http://localhost:8899 -X POST -H "Content-Type: application/json" -d '
  {"jsonrpc":"2.0","id":1, "method":"getSlot"}
'