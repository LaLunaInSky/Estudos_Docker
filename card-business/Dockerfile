# image python 

FROM python:3.12
WORKDIR /app
COPY requirements.txt /app
RUN python -m venv venv
RUN /bin/bash -c "source venv/bin/activate && pip install --no-cache-dir -r requirements.txt"
COPY . /app
EXPOSE 8080
CMD /bin/bash -c "source venv/bin/activate && python app.py"


# image ubuntu

# FROM ubuntu:24.04
# RUN apt-get update && \
#     apt-get install python3.12 \
#     python3.12-dev python3.12-venv \
#     python3-pip -y 
# WORKDIR /app
# COPY . /app
# RUN python3 -m venv venv
# RUN /bin/bash -c "source venv/bin/activate && pip3 install --no-cache-dir -r requirements.txt"
# EXPOSE 8080
# CMD /bin/bash -c "source venv/bin/activate && python3 app.py"