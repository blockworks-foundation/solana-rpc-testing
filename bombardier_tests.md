# Validator RPC tests with Bombardier

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

### jsonrpc_actix branch

The getBalance request return an error no test can be done on this request:

```
└─> curl http://192.168.88.12:8899 -X POST -H "Content-Type: application/json" -d '
>   {
>     "jsonrpc": "2.0",
>     "id": 1,
>     "method": "getBalance",
>     "params": [
>       "4EVAJ81v9fLjz2wp44SZmxgTbvYB8dBRZ6s1SAqP99eZ"
>     ]
>   }
> '
{"jsonrpc":"2.0","error":{"code":-32602,"message":"Invalid params: invalid type: null, expected struct RpcContextConfig"},"id":1}

```


## Timeout 

When the number of concurrent connection increase, timeout appears in the result. The limit of timeout arrival is:
 - master: 2800
 - Jsonrpc: 2500

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

## Conclusion

The tokio_jsonrpsee show a little improvement for both request compared to master branch.

For the getBlocks the jsonrpc_actix show an important improvement of the performance. I've to find the reason because I don't think it's only related to the json serialisation and HTTP request handling. This improvement mus tbe confirmed with the getBalance request when it will be available.

## Actions

 - correct getBalance on jsonrpc_actix
 - Do the getBalance test for jsonrpc_actix branch
 - See the reason of the performance difference between jsonrpc_actix branch and the others.