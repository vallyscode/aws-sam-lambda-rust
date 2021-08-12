# Rust lambda custom runtime

Simple example of Rust lambda deployment.

## Template
SAM can build custom runtime for us, but with some additional guidance from out side.

We need to define a type of runtime we want to use by setting its value to one of **provided.al2**(Amazon Linux 2 (AL2)) or **provided**(Amazon Linux).

Next thing SAM needs to know is how to build our custom runtime, for this we must declare **Metadata** attribute with specified build method as **makefile**.

Our **Makefile** must include a steps to build the runtime for each custom lambda resource with following format **build-FUNCTION-LOGICAL-ID**.

Each lambda resource can have makefile located in different place, to set a location we need to provide **CodeUri** with relative path to **Makefile**.

#### Example:
```yaml
  HeyFunction:
    Type: AWS::Serverless::Function
    Properties:
      Description: Says hey
      CodeUri: .
      Handler: bootstrap
      Runtime: provided.al2
      Tracing: Active
      Events:
        Greetings:
          Type: Api
          Properties:
            RestApiId: !Ref RestApi
            Path: /hey
            Method: GET
      Tags:
        Environment: !Ref Env
        Application: !Ref App
    Metadata:
      BuildMethod: makefile
```
#### Notes
> In this example **CodeUri** is set to **.** it means that makefile is located in the same place with template.

## Makefile
We must define a steps to build our Rust lambdas, as mentioned earlier target names must match to resource names from template.
```Makefile
build:
	rustup target add x86_64-unknown-linux-musl
	cargo build --target x86_64-unknown-linux-musl --release
	sam build

build-HeyFunction:
	cp ./target/x86_64-unknown-linux-musl/release/hey $(ARTIFACTS_DIR)/bootstrap
	strip $(ARTIFACTS_DIR)/bootstrap
```
#### Notes
> In this example all binaries are built during general build target execution and each **build-*** is just moving them to destination folders with striping.