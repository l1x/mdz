# Packages that are necessary for this build:
#   - rust (cargo)
#   - cargo-lambda
#   - zig
#   - aws cli (preferrably v2)
#   - zip
#   - shasum
#   - base64

# Build

FROM datadeft:cargo-lambda
USER root
RUN mkdir /opt/cache-manager && chown deploy /opt/cache-manager
USER deploy
WORKDIR /opt/cache-manager
COPY . .
RUN id
RUN cargo lambda build --release --arm64
RUN cd target/lambda/cache-manager/ && zip lambda.zip bootstrap
RUN cd target/lambda/cache-manager/ && sha256sum lambda.zip | base64 > lambda.zip.base64sha256

# Upload to artifact store

FROM datadeft:aws-cli-v2
ARG PROJECT
ARG STAGE
ARG VERSION
ARG AWS_ACCESS_KEY_ID
ARG AWS_SECRET_ACCESS_KEY
RUN echo "$PROJECT :: $STAGE :: $VERSION :: $AWS_ACCESS_KEY_ID"
USER root
RUN mkdir /opt/cache-manager && chown deploy /opt/cache-manager
USER deploy
WORKDIR /opt/cache-manager
COPY --from=0 /opt/cache-manager/target/lambda/cache-manager/lambda.zip .
COPY --from=0 /opt/cache-manager/target/lambda/cache-manager/lambda.zip.base64sha256 .
ENV AWS_REGION=eu-west-1
RUN aws sts get-caller-identity
RUN aws s3 cp lambda.zip s3://$PROJECT-$STAGE/etl/cache-manager/$VERSION/lambda.zip
RUN aws s3 cp --content-type text/plain lambda.zip.base64sha256 s3://$PROJECT-$STAGE/etl/cache-manager/$VERSION/lambda.zip.base64sha256
