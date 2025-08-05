from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class RetransmitBase(BaseModel):
    class Meta:
        name = "retransmit_base"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
