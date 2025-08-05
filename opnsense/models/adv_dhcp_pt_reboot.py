from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class AdvDhcpPtReboot(BaseModel):
    class Meta:
        name = "adv_dhcp_pt_reboot"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
