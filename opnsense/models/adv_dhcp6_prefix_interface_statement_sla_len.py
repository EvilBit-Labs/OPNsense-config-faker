from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class AdvDhcp6PrefixInterfaceStatementSlaLen(BaseModel):
    class Meta:
        name = "adv_dhcp6_prefix_interface_statement_sla_len"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
