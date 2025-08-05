from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class AdvDhcp6InterfaceStatementRequestOptions(BaseModel):
    class Meta:
        name = "adv_dhcp6_interface_statement_request_options"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
