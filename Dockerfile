FROM python:3.8
COPY ./ /srv/these-waifus-really-dont-exist/
WORKDIR /srv/these-waifus-really-dont-exist/
RUN mkdir img
RUN pip install -r requirements.txt
EXPOSE 5002
CMD python ./src/main.py