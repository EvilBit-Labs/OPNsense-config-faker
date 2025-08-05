from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class AdvDhcp6KeyInfoStatementRealm(BaseModel):
    class Meta:
        name = "adv_dhcp6_key_info_statement_realm"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
