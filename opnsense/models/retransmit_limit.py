from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class RetransmitLimit(BaseModel):
    class Meta:
        name = "retransmit_limit"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
