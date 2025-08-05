from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class AdvDhcpSendOptions(BaseModel):
    class Meta:
        name = "adv_dhcp_send_options"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
