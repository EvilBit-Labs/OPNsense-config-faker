from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Subscription(BaseModel):
    class Meta:
        name = "subscription"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
