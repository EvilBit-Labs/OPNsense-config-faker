from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Regdhcpdomain(BaseModel):
    class Meta:
        name = "regdhcpdomain"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
