from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class AdvDhcpPtSelectTimeout(BaseModel):
    class Meta:
        name = "adv_dhcp_pt_select_timeout"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
