from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class AdvDhcp6IdAssocStatementAddressPltime(BaseModel):
    class Meta:
        name = "adv_dhcp6_id_assoc_statement_address_pltime"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
