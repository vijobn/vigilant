FROM	node:18
#FROM	debian:latest
MAINTAINER Vijo Cherian codervijo@gmail.com

RUN	apt-get -y update && apt-get -y upgrade
RUN apt-get -y install git vim wget nodejs
RUN	apt-get -y install procps net-tools ssh
RUN     apt-get -y install curl

RUN mkdir /usr/src/app

WORKDIR /usr/src/app

# Install Node
#ENV NODE_VERS_MAJOR  16
#ENV NODE_VERS        v${NODE_VERS_MAJOR}.2.0
#RUN mkdir            /usr/src/app/node/
#RUN wget -O-         https://raw.githubusercontent.com/creationix/nvm/v0.39.0/install.sh | bash
#RUN chmod ugo+x      /root/.nvm/nvm.sh
#ENV NVM_DIR          /usr/src/app/nod
#RUN  . /root/.nvm/nvm.sh && nvm install ${NODE_VERS_MAJOR} && \
#	                        nvm use ${NODE_VERS_MAJOR} && \
#	                        export NVM_PATH=$(nvm current); \
#	                        echo "export NVM_PATH=$NVM_PATH" >> /root/.bashrc \
#	                        echo "export PATH=${NVM_DIR}/versions/node/$NVM_PATH/bin/:$PATH">>/root/.bashrc \
#				npm install -g @tauri-apps/cli

# For https://github.com/webpack/webpack/issues/14532#issuecomment-947012063
# ENV NODE_OPTIONS --openssl-legacy-provider

RUN     apt-get -y update && apt-get install -y curl
RUN     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs >/root/rustup.sh
RUN     apt-get -y install procps net-tools ssh
RUN     apt-get -y install openssl
RUN     apt-get -y install libssl-dev pkg-config
RUN     apt-get -y install build-essential
RUN apt-get -y install libxml2 libxml2-dev cmake libcurl4-openssl-dev
RUN apt-get -y install protobuf-compiler libprotobuf-dev
RUN apt-get -y install net-tools vim file
RUN /bin/bash -x /root/rustup.sh -y

RUN apt-get -y install libsoup*
#RUN apt-get -y install javascriptcoregtk-4.0 libpango* gdk-pixbuf-2.0
RUN apt-get -y install javascriptcoregtk-4.0 libpango*
RUN apt-get -y install libatk* gdk-3.0 libgdk* libgtk-3-dev
#RUN apt-get -y install webkit2gt* webkit2git-4.0

RUN apt-get update && apt-get install -y \
    libwebkit2gtk-4.0-dev \
    libgtk-3-dev \
    libgdk-pixbuf2.0-dev \
    libpango1.0-dev \
    libharfbuzz-dev \
    libatk1.0-dev \
    libcairo2-dev \
    libsoup2.4-dev \
    libjavascriptcoregtk-4.0-dev \
    libglib2.0-dev

RUN npm install -g @tauri-apps/cli


#RUN     npm install 
#RUN     npm install react-scripts@3.0.1 -g --silent

#CMD	["npm", "start"]
