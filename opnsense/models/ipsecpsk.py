from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Ipsecpsk(BaseModel):
    class Meta:
        name = "ipsecpsk"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
