FROM postgres:15.1-alpine3.17

WORKDIR /work

RUN apk update && apk add curl
RUN curl -L https://github.com/k0kubun/sqldef/releases/download/v0.15.0/psqldef_linux_amd64.tar.gz -o psqldef.tar.gz \
    && tar -zxvf psqldef.tar.gz \
    && mv psqldef /usr/local/bin \
    && rm psqldef.tar.gz

RUN echo 'alias psqldef="psqldef --user=$DB_USER --password=$DB_PASSWORD --host=$DB_HOST --port=$DB_PORT $DB_NAME"' >> ~/.bashrc
