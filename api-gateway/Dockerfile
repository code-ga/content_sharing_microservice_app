FROM golang:alpine as golang-compiler
WORKDIR /home/container
COPY . .
RUN go mod tidy
RUN go get .
RUN go build -o ./main.exe .

FROM golang:alpine as golang-remover
WORKDIR /home/container
COPY --from=golang-compiler /home/container ./


FROM alpine
WORKDIR /home/container

COPY --from=golang-remover /home/container ./
EXPOSE 8080
CMD [ "/home/container/main.exe" ]