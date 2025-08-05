from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class RetransmitTries(BaseModel):
    class Meta:
        name = "retransmit_tries"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
