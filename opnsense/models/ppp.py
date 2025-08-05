from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Ppp(BaseModel):
    class Meta:
        name = "ppp"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
