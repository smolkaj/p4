# P4Runtime Server End-to-End Tests

## GRPC Reflection

```console
$ grpcurl -plaintext "[::1]:9559" list
grpc.reflection.v1.ServerReflection
p4.v1.P4Runtime

$ grpcurl -plaintext "[::1]:9559" list p4.v1.P4Runtime
p4.v1.P4Runtime.Capabilities
p4.v1.P4Runtime.GetForwardingPipelineConfig
p4.v1.P4Runtime.Read
p4.v1.P4Runtime.SetForwardingPipelineConfig
p4.v1.P4Runtime.StreamChannel
p4.v1.P4Runtime.Write

```

## Capabilities

```console
$ grpcurl -plaintext "[::1]:9559" p4.v1.P4Runtime.Capabilities
{
  "p4runtimeApiVersion": "1.4.1"
}

```

## Config

```console
$ grpcurl -plaintext "[::1]:9559" p4.v1.P4Runtime.GetForwardingPipelineConfig
? 73
ERROR:
  Code: FailedPrecondition
  Message: no pipeline configured

$ grpcurl -plaintext -d '{"device_id": 42, "config": {"p4info": {}}}' "[::1]:9559" p4.v1.P4Runtime.SetForwardingPipelineConfig
{}

$ grpcurl -plaintext "[::1]:9559" p4.v1.P4Runtime.GetForwardingPipelineConfig
{
  "config": {
    "p4info": {}
  }
}

```

## Write

```console
$ echo $P4INFO
? 0
$ grpcurl -plaintext "[::1]:9559" p4.v1.P4Runtime.Write
{}

```



