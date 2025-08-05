from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class RetransmitTimeout(BaseModel):
    class Meta:
        name = "retransmit_timeout"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
