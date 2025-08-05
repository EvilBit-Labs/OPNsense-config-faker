from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class AdvDhcpPtInitialInterval(BaseModel):
    class Meta:
        name = "adv_dhcp_pt_initial_interval"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
