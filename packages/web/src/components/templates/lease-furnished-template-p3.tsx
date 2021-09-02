import { Alert, Pane } from "evergreen-ui";
import React from "react";
import { DocumentPageProps } from "../document/document";

export const LeaseFurnishedTemplatePart3: React.FunctionComponent<
  DocumentPageProps
> = ({
  form,
  getItem,
}) => {
  return (
    <Pane>
      <h2 id="v-travaux">V. TRAVAUX</h2>
      <p>
        <strong>A.</strong>
        Le cas échéant, Montant et nature des travaux d'amélioration ou de mise
        en conformité avec les caractéristiques de décence effectués depuis la
        fin du dernier contrat de location ou depuis le dernier renouvellement
        (36) :
      </p>
      {getItem("contract_works_decence_since_last_rental")}
      <p>
        <strong>B.</strong>
        Le cas échéant, Majoration du loyer en cours de bail consécutive à des
        travaux d'amélioration entrepris par le bailleur ou d'acquisitions
        d'équipements (37) :
      </p>
      {getItem("contract_works_rent_increase_lender")}
      <p>
        <strong>C.</strong>
        Le cas échéant, Diminution de loyer en cours de bail consécutive à des
        travaux entrepris par le locataire :
      </p>
      {getItem("contract_works_rent_decrease_tenant")}
      <h2 id="vi-garanties">VI. GARANTIES</h2>
      <p>
        Le cas échéant, Montant du dépôt de garantie de l'exécution des
        obligations du locataire :
      </p>
      {getItem("contract_deposit_amount")}
      <h2 id="vii-le-cas-echeant-clause-de-solidarite">
        VII. LE CAS ECHEANT, CLAUSE DE SOLIDARITE
      </h2>
      <p>
        Modalités particulières des obligations en cas de pluralité de
        locataires :
      </p>
      {getItem("contract_solidarity_clause")}
      <h2 id="viii-le-cas-echeant-clause-resolutoire">
        VIII. LE CAS ECHEANT, CLAUSE RESOLUTOIRE
      </h2>
      <p>
        Modalités de résiliation de plein droit du contrat :
      </p>
      {getItem("contract_resolutary_clause")}
      <h2 id="ix-le-cas-echeant-honoraires-de-location-38-">
        IX. LE CAS ECHEANT, HONORAIRES DE LOCATION (38)
      </h2>
      <p>
        <strong>A.</strong> Dispositions applicables
      </p>
      <Alert
        intent="none"
        marginBottom={32}
      >
        Il est rappelé les dispositions du I de l'article 5 de la loi du 6
        juillet 1989, alinéas 1 à 3 : La rémunération des personnes mandatées
        pour se livrer ou prêter leur concours à l'entremise ou à la négociation
        d'une mise en location d'un logement, tel que défini aux articles 2 et
        25-3, est à la charge exclusive du bailleur, à l'exception des
        honoraires liés aux prestations mentionnées aux deuxième et troisième
        alinéas du présent I. Les honoraires des personnes mandatées pour
        effectuer la visite du preneur, constituer son dossier et rédiger un
        bail sont partagés entre le bailleur et le preneur. Le montant toutes
        taxes comprises imputé au preneur pour ces prestations ne peut excéder
        celui imputé au bailleur et demeure inférieur ou égal à un plafond par
        mètre carré de surface habitable de la chose louée fixé par voie
        réglementaire et révisable chaque année, dans des conditions définies
        par décret. Ces honoraires sont dus à la signature du bail. Les
        honoraires des personnes mandatées pour réaliser un état des lieux sont
        partagés entre le bailleur et le preneur. Le montant toutes taxes
        comprises imputé au locataire pour cette prestation ne peut excéder
        celui imputé au bailleur et demeure inférieur ou égal à un plafond par
        mètre carré de surface habitable de la chose louée fixé par voie
        réglementaire et révisable chaque année, dans des conditions définies
        par décret. Ces honoraires sont dus à compter de la réalisation de la
        prestation. Plafonds applicables :
      </Alert>
      <ul>
        <li>
          montant du plafond des honoraires imputables aux locataires en matière
          de prestation de visite du preneur, de constitution de son dossier et
          de rédaction de bail :
          {getItem("contract_tenant_fee_cap_new_rental")}
          €/m2 de surface habitable ;
        </li>
        <li>
          montant du plafond des honoraires imputables aux locataires en matière
          d'établissement de l'état des lieux d'entrée :

          {getItem("contract_tenant_fee_cap_report_by_meter")}
          €/m2 de surface habitable.
        </li>
      </ul>
      <p>
        <strong>B.</strong> Détail et répartition des honoraires
      </p>
      <p>
        <strong>1. Honoraires à la charge du bailleur :</strong>
      </p>
      <ul>
        <li>
          prestations de visite du preneur, de constitution de son dossier et de
          rédaction de bail :
          {getItem("contract_lender_fee_cap_prestations")}
        </li>
        <li>
          le cas échéant, Prestation de réalisation de l'état des lieux d'entrée
          :
          {getItem("contract_lender_fee_cap")}
        </li>
        <li>
          le cas échéant, Autres prestations :
          {getItem("contract_lender_fee_cap_other")}
        </li>
      </ul>
      <p>
        <strong>2. Honoraires à la charge du locataire :</strong>
      </p>
      <ul>
        <li>
          prestations de visite du preneur, de constitution de son dossier et de
          rédaction de bail :
          {getItem("contract_tenant_fee_cap_prestations")}
        </li>
        <li>
          le cas échéant, Prestation de réalisation de l'état des lieux d'entrée
          :
          {getItem("contract_tenant_fee_cap_report_prestations")}
        </li>
      </ul>
      <h2 id="x-autres-conditions-particulieres">
        X. AUTRES CONDITIONS PARTICULIERES
      </h2>
      {getItem("contract_other_conditions")}
      <h2 id="xi-xi-annexes">XI. ANNEXES</h2>
      <p>
        Sont annexées et jointes au contrat de location les pièces suivantes :
      </p>
      <p>
        <strong>A.</strong>
        Le cas échéant, un extrait du règlement concernant la destination de
        l'immeuble, la jouissance et l'usage des parties privatives et communes,
        et précisant la quote-part afférente au lot loué dans chacune des
        catégories de charges
      </p>
      <p>
        <strong>B.</strong> Un dossier de diagnostic technique comprenant
      </p>
      <ul>
        <li>un diagnostic de performance énergétique ;</li>
        <li>
          un constat de risque d'exposition au plomb pour les immeubles
          construits avant le 1er janvier 1949 ;
        </li>
        <li>
          le cas échéant, une copie d'un état mentionnant l'absence ou la
          présence de matériaux ou de produits de la construction contenant de
          l'amiante (39) ;
        </li>
        <li>
          le cas échéant, Un état de l'installation intérieure d'électricité et
          de gaz, dont l'objet est d'évaluer les risques pouvant porter atteinte
          à la sécurité des personnes (40) ;
        </li>
        <li>
          le cas échéant, un état des risques naturels et technologiques pour le
          zones couvertes par un plan de prévention des risques technologiques
          ou par un plan de prévention des risques naturels prévisibles,
          prescrit ou approuvé, ou dans des zones de sismicité (41).
        </li>
      </ul>
      <p>
        <strong>C.</strong>{" "}
        Une notice d'information relative aux droits et obligations des
        locataires et des bailleurs
      </p>
      <p>
        <strong>D.</strong>{" "}
        Un état des lieux, un inventaire et un état détaillé du mobilier (42)
      </p>
      <p>
        <strong>E.</strong>{" "}
        Le cas échéant, une autorisation préalable de mise en location (43)
      </p>
      <p>
        <strong>F.</strong>{" "}
        Le cas échéant, Les références aux loyers habituellement constatés dans
        le voisinage pour des logements comparables (44)
      </p>
      <p>Signature du bailleur</p>
      <br />
      <br />
      <p>Signature du locataire</p>
      <br />
      <br />
      <h1 id="contrat-type-issu-de-l-annexe-2-du-d-cret-du-29-mai-2015">
        Contrat type issu de l’annexe 2 du décret du 29 mai 2015
      </h1>
      <Alert
        intent="none"
        marginBottom={32}
      >
        (24) Préciser si la personne morale est une société civile constituée
        exclusivement entre parents et alliés jusqu'au quatrième degré inclus.
        (25) A reproduire si pluralité de bailleur. (26) Mention obligatoire
        s'appliquant aux professionnels exerçant une activité mentionnée à
        l'article 1er de la loi n° 70- 9 du 2 janvier 1970 réglementant les
        conditions d'exercice des activités relatives à certaines opérations
        portant sur les immeubles et les fonds de commerce. (27) Si chauffage
        collectif, préciser les modalités de répartition de la consommation du
        locataire. (28) En cas de production collective, préciser les modalités
        de répartition de la consommation du locataire. (29) Lorsqu'un
        complément de loyer est appliqué, le loyer mensuel s'entend comme la
        somme du loyer de base et de ce complément. (30) Zones d'urbanisation
        continue de plus de 50 000 habitants où il existe un déséquilibre marqué
        entre l'offre et la demande de logements, entraînant des difficultés
        sérieuses d'accès au logement sur l'ensemble du parc résidentiel telles
        que définies par décret. (31) Mention obligatoire si le précédent
        locataire a quitté le logement moins de dix-huit mois avant la signature
        du bail. (32) Si les parties conviennent d'un forfait de charges et de
        sa révision annuelle, ce forfait est révisé dans les mêmes conditions
        que le loyer principal. (33) Au cours de l'exécution du contrat de
        location et dans les conditions prévues par la loi, les colocataires
        peuvent provoquer la résiliation de l'assurance souscrite par le
        bailleur pour leur compte. (34) Correspond au montant de la prime
        d'assurance annuelle, éventuellement majoré dans la limite d'un montant
        fixé par décret en Conseil d'Etat. (35) Paiement mensuel de droit à tout
        moment à la demande du locataire. (36) Le cas échéant, préciser par
        ailleurs le montant des travaux d'amélioration effectués au cours des
        six dernier mois. (37) Clause invalide pour les travaux de mise en
        conformité aux caractéristiques de décence. (38) A mentionner lorsque le
        contrat de location est conclu avec le concours d'une personne mandatée
        et rémunérée à cette fin. (39) A compter de l'entrée en vigueur du
        décret d'application lisant notamment les matériaux ou produits
        concernés. (40) A compter de la date d'entrée en vigueur de cette
        disposition, prévue par décret. (41) La liste des communes comprises
        dans ces zones est définie localement par arrêté préfectoral. (42) Ces
        documents sont établis lors de la remise des clés, dont la date peut
        être ultérieure à celle de conclusion du contrat. (43) Dispositif
        applicable dans certains territoires présentant une proportion
        importante d'habitat dégradé délimité localement par l'établissement
        public de coopération intercommunale compétent en matière d'habitat ou,
        à défaut, le conseil municipal (art. 92 de la loi n° 2014-366 du 24 mars
        2014 pour l'accès au logement et un urbanisme rénové). (44) Lorsque la
        détermination du montant du loyer est la conséquence d'une procédure
        liée au fait que le loyer précédemment appliqué était manifestement
        sous-évalué.
      </Alert>
    </Pane>
  );
};
