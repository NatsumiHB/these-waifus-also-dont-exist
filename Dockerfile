FROM python:3.8
COPY ./ /srv/this-waifu-redir/
WORKDIR /srv/this-waifu-redir/
RUN mkdir img
RUN pip install -r requirements.txt
EXPOSE 5002
CMD python ./src/main.py