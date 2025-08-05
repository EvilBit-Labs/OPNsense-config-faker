from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Regdhcpstatic(BaseModel):
    class Meta:
        name = "regdhcpstatic"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
