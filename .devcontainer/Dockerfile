FROM rust:latest

RUN apt-get update \
	&& apt-get -y install --fix-missing \
		vim \
		bash-completion \
		sudo \
		locales \
	&& apt-get clean

RUN sed -i 's/^\s*#\s*\(en_US.UTF-8\)/\1/' /etc/locale.gen \
	&& locale-gen \
	&& update-locale LANG=en_US.UTF-8

#Add a container user
RUN addgroup builder && useradd -ms /bin/bash -g builder builder

# grant passwordless sudo for ALL commands
RUN mkdir -p /etc/sudoers.d \
	&& echo "builder ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/builder \
	&& chmod 0440 /etc/sudoers.d/builder

USER builder
RUN rustup component add rustfmt

WORKDIR /workspace
