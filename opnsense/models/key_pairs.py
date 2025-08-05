from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class KeyPairs(BaseModel):
    class Meta:
        name = "keyPairs"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
