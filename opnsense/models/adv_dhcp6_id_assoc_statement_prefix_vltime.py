from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class AdvDhcp6IdAssocStatementPrefixVltime(BaseModel):
    class Meta:
        name = "adv_dhcp6_id_assoc_statement_prefix_vltime"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
