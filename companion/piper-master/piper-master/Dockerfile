FROM debian:bullseye as build
ARG TARGETARCH
ARG TARGETVARIANT

ENV LANG C.UTF-8
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install --yes --no-install-recommends \
        build-essential cmake ca-certificates curl pkg-config git sox wget

#Install ollama
#RUN curl -fsSL https://ollama.com/install.sh | sh
WORKDIR /build

COPY ./ ./
RUN cmake -Bbuild -DCMAKE_INSTALL_PREFIX=install
RUN cmake --build build --config Release
RUN cmake --install build

# Do a test run
RUN ./build/piper --help

# Build .tar.gz to keep symlinks
WORKDIR /dist
#COPY ./en_GB-semaine-medium.onnx /dist


#install Text to speech
COPY en_GB-semaine-medium.onnx .
COPY en_GB-semaine-medium.onnx.json .
#RUN ollama pull llama3.2:latest
RUN mkdir -p piper && \
    cp -dR /build/install/* ./piper/ && \
    tar -czf "piper_${TARGETARCH}${TARGETVARIANT}.tar.gz" piper/
COPY script.sh .


EXPOSE 69
RUN apt-get install --yes netcat
RUN apt-get update && \
    apt-get install -y pulseaudio && \
    apt-get install -y alsa-utils && \
            apt-get clean
RUN apt-get install libssl-dev -y
RUN apt-get install pkg-config -y


#CMD nc -l -p 69 -k | ./piper/piper --cuda --model en_GB-semaine-medium.onnx --output-raw |   aplay -r 22050 -f S16_LE -t raw -
CMD while true; do nc -l -p 69 | ./piper/piper --cuda --model en_GB-semaine-medium.onnx --output_file reply.wav; play reply.wav; done

#WORKDIR /root/.ollama/models/manifests/registry.ollama.ai/library
#RUN test -e model || wget https://huggingface.co/deepseek-ai/DeepSeek-R1/resolve/main/model-00001-of-000163.safetensors?download=true -O model
# -----------------------------------------------------------------------------

# FROM debian:bullseye as test
# ARG TARGETARCH
# ARG TARGETVARIANT

# WORKDIR /test

# COPY local/en-us/lessac/low/en-us-lessac-low.onnx \
#      local/en-us/lessac/low/en-us-lessac-low.onnx.json ./

# # Run Piper on a test sentence and verify that the WAV file isn't empty
# COPY --from=build /dist/piper_*.tar.gz ./
# RUN tar -xzf piper*.tar.gz
# RUN echo 'This is a test.' | ./piper/piper -m en-us-lessac-low.onnx -f test.wav
# RUN if [ ! -f test.wav ]; then exit 1; fi
# RUN size="$(wc -c < test.wav)"; \
#     if [ "${size}" -lt "1000" ]; then echo "File size is ${size} bytes"; exit 1; fi

# -----------------------------------------------------------------------------

#FROM scratch

# COPY --from=test /test/piper_*.tar.gz /test/test.wav ./
#COPY --from=build /dist/piper_*.tar.gz ./
