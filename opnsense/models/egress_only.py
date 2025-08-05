from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class EgressOnly(BaseModel):
    class Meta:
        name = "egress_only"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
