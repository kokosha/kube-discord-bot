EXECUTABLE_NAME = kube-discord-bot
SERVICE_NAME = kube-discord-bot.service
SYSTEMD_DIR = /etc/systemd/system

.PHONY: all permission add-service check-service remove-service

all: add-service

permission:
	sudo chmod +x $(EXECUTABLE_NAME)

add-service: permission
	sudo cp $(SERVICE_NAME) $(SYSTEMD_DIR)/
	sudo systemctl daemon-reload
	sudo systemctl enable $(SERVICE_NAME)
	sudo systemctl restart $(SERVICE_NAME)

check-service:
	sudo systemctl status $(SERVICE_NAME)

remove-service:
	sudo systemctl stop $(SERVICE_NAME)
	sudo systemctl disable $(SERVICE_NAME)
	sudo rm -f $(SYSTEMD_DIR)/$(SERVICE_NAME)
	sudo systemctl daemon-reload