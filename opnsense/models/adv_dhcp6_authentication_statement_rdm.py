from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class AdvDhcp6AuthenticationStatementRdm(BaseModel):
    class Meta:
        name = "adv_dhcp6_authentication_statement_rdm"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
