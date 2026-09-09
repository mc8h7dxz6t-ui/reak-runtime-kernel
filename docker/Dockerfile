FROM python:3.11-slim-bookworm

ENV PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1 \
    PIP_NO_CACHE_DIR=1 \
    PIP_DISABLE_PIP_VERSION_CHECK=1

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY credit-duty/requirements-dev.txt /tmp/cd-requirements-dev.txt
RUN pip install -r /tmp/cd-requirements-dev.txt

COPY credit-duty/ credit-duty/
COPY demo/ demo/
COPY scripts/docker-test.sh scripts/docker-test.sh

RUN chmod +x scripts/docker-test.sh

CMD ["./scripts/docker-test.sh"]
