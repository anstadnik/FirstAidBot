FROM python:3.10.2

ADD . .

RUN python -m pip install -r requirements.txt

#TODO add only source code not everything

CMD ["python3", "bot.py"]