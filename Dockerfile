FROM python:3.8
COPY ./ /srv/these-waifus-really-dont-exist/
WORKDIR /srv/these-waifus-really-dont-exist/
RUN pip install -r requirements.txt
EXPOSE 5002
WORKDIR ./src/
CMD python ./main.py